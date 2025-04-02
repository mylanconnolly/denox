use deno_core::v8;
use deno_core::JsRuntime;
use rustler::{Atom, Encoder, Env, Error, Term};

// Initialize V8 once per process
static INIT: std::sync::Once = std::sync::Once::new();

#[rustler::nif]
fn eval_js<'a>(env: Env<'a>, code: String, bindings: Term<'a>) -> Result<Term<'a>, Error> {
    // Initialize V8 only once
    INIT.call_once(|| {
        // This ensures V8 is initialized only once per process
        let platform = deno_core::v8::new_default_platform(0, false).make_shared();
        deno_core::v8::V8::initialize_platform(platform);
        deno_core::v8::V8::initialize();
    });
    
    let mut runtime = JsRuntime::new(Default::default());
    let scope = &mut runtime.handle_scope();
    let context = scope.get_current_context();
    let global = context.global(scope);

    // Convert Elixir bindings to JavaScript values
    let iter = bindings.decode::<rustler::MapIterator>()?;
    for (key, value) in iter {
        let key_str: String = key.decode()?;
        let js_key = v8::String::new(scope, &key_str)
            .ok_or_else(|| Error::Term(Box::new("Failed to create JavaScript key")))?;

        let js_value = term_to_v8_value(scope, value)?;
        global.set(scope, js_key.into(), js_value);
    }

    // Execute the JavaScript code within the same scope
    let script = v8::String::new(scope, &code)
        .ok_or_else(|| Error::Term(Box::new("Failed to create JavaScript code")))?;
    let script = v8::Script::compile(scope, script, None)
        .ok_or_else(|| Error::Term(Box::new("Failed to compile script")))?;
    let result = script
        .run(scope)
        .ok_or_else(|| Error::Term(Box::new("Failed to run script")))?;

    // Convert the result to an Elixir term
    v8_value_to_term(env, scope, result)
}

fn v8_value_to_term<'a>(
    env: Env<'a>,
    scope: &mut v8::HandleScope,
    value: v8::Local<v8::Value>,
) -> Result<Term<'a>, Error> {
    if value.is_boolean() {
        // Convert to boolean object and get its value
        let boolean = v8::Local::<v8::Boolean>::try_from(value)
            .map_err(|_| Error::Term(Box::new("Failed to cast to boolean")))?;
        let bool_val = boolean.boolean_value(scope);
        Ok(bool_val.encode(env))
    } else if value.is_number() {
        let num = value
            .to_number(scope)
            .ok_or_else(|| Error::Term(Box::new("Failed to convert number")))?;

        // Get the actual float value
        let float_val = num.value();

        // Check if it's actually an integer by comparing with floor
        if float_val == float_val.floor() {
            // It's a whole number, convert to integer
            let int_val = float_val as i64;
            Ok(int_val.encode(env))
        } else {
            // It's a decimal number, keep as float
            Ok(float_val.encode(env))
        }
    } else if value.is_array() {
        let array = value
            .to_object(scope)
            .ok_or_else(|| Error::Term(Box::new("Failed to convert array")))?;
        // Cast the object to an array first
        let array = v8::Local::<v8::Array>::try_from(array)
            .map_err(|_| Error::Term(Box::new("Failed to cast to array")))?;
        let length = array.length();
        let mut result = Vec::with_capacity(length as usize);

        for i in 0..length {
            let item = array
                .get_index(scope, i)
                .ok_or_else(|| Error::Term(Box::new("Failed to get array item")))?;
            let term = v8_value_to_term(env, scope, item)?;
            result.push(term);
        }

        Ok(result.encode(env))
    } else if value.is_object() && !value.is_null() {
        let object = value
            .to_object(scope)
            .ok_or_else(|| Error::Term(Box::new("Failed to convert object")))?;
        let mut map = Term::map_new(env);

        let names = object
            .get_own_property_names(scope, v8::GetPropertyNamesArgs::default())
            .ok_or_else(|| Error::Term(Box::new("Failed to get object properties")))?;

        for i in 0..names.length() {
            let key = names
                .get_index(scope, i)
                .ok_or_else(|| Error::Term(Box::new("Failed to get property name")))?;
            let value = object
                .get(scope, key)
                .ok_or_else(|| Error::Term(Box::new("Failed to get property value")))?;

            let key_str = key
                .to_string(scope)
                .ok_or_else(|| Error::Term(Box::new("Failed to convert key to string")))?
                .to_rust_string_lossy(scope);
            let value_term = v8_value_to_term(env, scope, value)?;

            map = map.map_put(key_str.encode(env), value_term)?;
        }

        Ok(map)
    } else if value.is_string() {
        let string = value
            .to_string(scope)
            .ok_or_else(|| Error::Term(Box::new("Failed to convert string")))?;
        Ok(string.to_rust_string_lossy(scope).encode(env))
    } else if value.is_null() || value.is_undefined() {
        Ok(rustler::types::atom::nil().encode(env))
    } else {
        let string = value
            .to_string(scope)
            .ok_or_else(|| Error::Term(Box::new("Failed to convert value to string")))?;
        Ok(string.to_rust_string_lossy(scope).encode(env))
    }
}

fn term_to_v8_value<'a>(
    scope: &mut v8::HandleScope<'a>,
    term: Term,
) -> Result<v8::Local<'a, v8::Value>, Error> {
    if let Ok(b) = term.decode::<bool>() {
        return Ok(v8::Boolean::new(scope, b).into());
    }

    if term.is_number() {
        if let Ok(n) = term.decode::<f64>() {
            return Ok(v8::Number::new(scope, n).into());
        }
        if let Ok(n) = term.decode::<i64>() {
            return Ok(v8::Number::new(scope, n as f64).into());
        }
    }

    if let Ok(s) = term.decode::<String>() {
        if let Some(js_string) = v8::String::new(scope, &s) {
            return Ok(js_string.into());
        }
    }

    if let Ok(_atom) = term.decode::<Atom>() {
        // Convert atom to string directly using decode
        let atom_str: String = term.decode()?;
        if let Some(js_string) = v8::String::new(scope, &atom_str) {
            return Ok(js_string.into());
        }
    }

    if let Ok(list) = term.decode::<Vec<Term>>() {
        let js_array = v8::Array::new(scope, list.len() as i32);
        for (i, item) in list.iter().enumerate() {
            let value = term_to_v8_value(scope, *item)?;
            js_array.set_index(scope, i as u32, value);
        }
        return Ok(js_array.into());
    }

    if let Ok(iter) = term.decode::<rustler::MapIterator>() {
        let js_object = v8::Object::new(scope);
        for (key, value) in iter {
            let key_str: String = key.decode()?;
            let js_key = v8::String::new(scope, &key_str)
                .ok_or_else(|| Error::Term(Box::new("Failed to create JavaScript key")))?;
            let js_value = term_to_v8_value(scope, value)?;
            js_object.set(scope, js_key.into(), js_value);
        }
        return Ok(js_object.into());
    }

    Err(Error::Term(Box::new("Unsupported value type")))
}

rustler::init!("Elixir.Denox");

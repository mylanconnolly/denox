defmodule DenoxTest do
  use ExUnit.Case

  doctest Denox

  test "Runs simple code" do
    assert Denox.eval_js("1 + 1") == 2
  end

  test "Runs code with bindings" do
    assert Denox.eval_js("a + b", %{"a" => 1, "b" => 2}) == 3
  end

  test "casts arrays" do
    assert Denox.eval_js("[1, 2, 3]") == [1, 2, 3]
  end

  test "casts floats" do
    assert Denox.eval_js("3.14") == 3.14
  end

  test "casts integers" do
    assert Denox.eval_js("42") == 42
  end

  test "casts objects to maps" do
    # Wrap the object in parentheses to ensure it's parsed as an object and not
    # a block
    assert Denox.eval_js("({a: 1, b: 2})") == %{"a" => 1, "b" => 2}
  end
end

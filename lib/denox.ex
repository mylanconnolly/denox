defmodule Denox do
  @moduledoc """
  Documentation for `Denox`.
  """

  use Rustler, otp_app: :denox, crate: :denox

  def eval_js(_code, _bindings \\ %{}), do: :erlang.nif_error(:nif_not_loaded)
end

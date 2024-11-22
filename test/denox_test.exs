defmodule DenoxTest do
  use ExUnit.Case
  doctest Denox

  test "greets the world" do
    assert Denox.hello() == :world
  end
end

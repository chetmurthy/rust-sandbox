import pytest

from hello_pyo3 import *

def test1():
    t = Thing([("foo1",1), ("foo2",2), ("foo3",3), ("foo4",4)])
    assert t[0] == ("foo1",1)
    assert t[0:2] == [('foo1', 1), ('foo2', 2)]

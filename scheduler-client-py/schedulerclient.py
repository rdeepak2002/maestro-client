#!/usr/bin/env python
from schedulerclient_py import Workflow

if __name__ == '__main__':
    workflow = Workflow.builder().id("x").build()
    assert "x" == workflow.id
    print("Python tests pass")

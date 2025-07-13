<h1 align="center">🎻 maestro-client</h1>

<p align="center">
  Unofficial Python and Java client libraries for the <a href="https://github.com/Netflix/maestro">Maestro workflow scheduler</a>
</p>

---

## ✨ Features

- 📦 Python and Java clients
- 🔧 Easy workflow construction with builders
- 🔁 Seamless integration with Maestro scheduler

---

## 🧑‍🏫 Examples

### 🐍 Python Client

```python
from maestro_client import WorkflowBuilder, StepBuilder

# Define steps
step_1 = (
    StepBuilder("job.1", "NoOp")
    .add_transition("job.2", "true")
    .add_transition("job.3", "true")
    .build()
)

step_2 = (
    StepBuilder("job.2", "NoOp")
    .add_param_expression("bar", "return foo + 1;", "long")
    .build()
)

step_3 = (
    StepBuilder("job.3", "NoOp")
    .add_param_value("bar", "1", "long")
    .build()
)

# Build the workflow using defined steps
workflow_push_result = (
    WorkflowBuilder("sample-python-client-workflow")
    .add_param("foo", "return 1+1;", "long")
    .add_step(step_1)
    .add_step(step_2)
    .add_step(step_3)
    .push()
)

print(workflow_push_result)
```

### ☕ Java Client (JDK >= 23)

```java
// Coming soon...
```

> **Why JDK >= 23?** JDK 23 officially supports [JEP 454: Foreign Function & Memory API](https://openjdk.org/jeps/454)
> which is a modern alternative to JNI. Using JNI can be difficult and prone to mistakes. It also adds considerable
> overhead, as frequent native method invocations lack the performance benefits of Just-In-Time (JIT) compilation.
> If support for Java < 22 is needed via JNI, contributions are welcome.

---

## 🛠️ Build the Library

```bash
make build
```

> 🧠 This builds the clients for the platform you're on. When using
> the library, the `MAESTRO_CLIENT_API_URL` environment variable can
> be defined to specify where the Maestro API server is hosted.
> By default, port `8080` will be used.

> 📦 The library is outputted to <todo>

---

## 🧪 Run Integration Tests

```bash
make test
```

> 🧪 This runs integration tests for the clients.  
> ⚠️ Ensure Maestro is running on port 8080 before executing tests.

---

## 📁 Project Structure

- `scheduler-client/` — Library source code & Rust client
- `scheduler-client-jdk23/` — Java client that wraps the Rust library
- `scheduler-client-py/` — Python client that wraps the Rust library

---

## ❤️ Contributing

Contributions are welcome! Feel free to submit PRs, open issues, or suggest features.

---

<p align="center">
  Made with 🦀 by <a href="https://github.com/rdeepak2002">rdeepak2002</a>
</p>
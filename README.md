<h1 align="center">🎻 maestro-client</h1>

<p align="center">
  Unofficial Python and (coming soon) Kotlin client libraries for the <a href="https://github.com/Netflix/maestro">Maestro workflow scheduler</a>
</p>

---

## ✨ Features

- 📦 Python and (coming soon) Kotlin clients
- 🔧 Easy workflow construction with builders
- 🔁 Seamless integration with Maestro scheduler

---

## 🧪 Examples

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

### ☕ Kotlin Client

```kotlin
// Coming soon...
```

---

## 🛠️ Build the Library

```bash
./build.sh
```

> 🧠 This builds the clients for the platform you're on. When using
> the library, the `MAESTRO_CLIENT_API_URL` environment variable can
> be defined to specify where the Maestro API server is hosted.
> By default, port `8080` will be used.

> 📦 The library is outputted to `target/release/maestro_client.py`.

> ⚠️ When using the library, the `libmaestro_client.(so | dylib)` file
> must be present next to `maestro_client.py`.

---

## 🧪 Run Integration Tests

```bash
./run_integration_tests.sh
```

> 🧪 This runs integration tests for the clients.  
> ⚠️ Ensure Maestro is running on port 8080 before executing tests.

---

## 📁 Project Structure

- `src/` — Library source code
- `tests/` — Integration tests

---

## ❤️ Contributing

Contributions are welcome! Feel free to submit PRs, open issues, or suggest features.

---

<p align="center">
  Made with 🦀 by <a href="https://github.com/rdeepak2002">rdeepak2002</a>
</p>

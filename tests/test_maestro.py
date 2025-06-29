import os
import json
import logging
from maestro_client import WorkflowBuilder, StepBuilder


def test_workflow_push():
    logging.info(f"Starting workflow creation test...")

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

    logging.info(f"Successfully pushed workflow: {workflow_push_result}")

    # Get the path to expected.json relative to this test file
    current_dir = os.path.dirname(os.path.abspath(__file__))
    expected_json_path = os.path.join(current_dir, "expected.json")

    # Read and parse the expected JSON
    with open(expected_json_path, 'r') as f:
        expected_json = json.load(f)

    # Parse the actual response text
    actual_json = json.loads(workflow_push_result.text)

    # Get workflow definitions
    actual_def = actual_json["workflow_definition"]
    expected_def = expected_json["workflow_definition"]

    # Ignore dynamic fields
    actual_def["properties_snapshot"].pop("create_time")
    expected_def["properties_snapshot"].pop("create_time")
    actual_def["metadata"].pop("workflow_version_id")
    expected_def["metadata"].pop("workflow_version_id")
    actual_def["metadata"].pop("create_time")
    expected_def["metadata"].pop("create_time")
    actual_def.pop("activate_time")
    expected_def.pop("activate_time")
    actual_def.pop("modify_time")
    expected_def.pop("modify_time")

    # Compare the JSONs
    assert actual_json == expected_json, "Workflow response doesn't match expected JSON"

    logging.info("\033[92mPython integration tests PASSED\033[0m")


if __name__ == "__main__":
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s - %(levelname)s - %(message)s'
    )
    test_workflow_push()

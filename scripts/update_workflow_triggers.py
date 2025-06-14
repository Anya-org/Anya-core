#!/usr/bin/env python3
import os
import yaml

# Standardized triggers for all workflows
standard_triggers = {
    'push': {
        'branches': ['main', 'release/*']
    },
    'pull_request': {
        'branches': ['main']
    },
    'workflow_dispatch': {}
}

workflows_dir = os.path.join(os.getcwd(), '.github', 'workflows')
for fname in os.listdir(workflows_dir):
    if not (fname.endswith('.yml') or fname.endswith('.yaml')):
        continue
    path = os.path.join(workflows_dir, fname)
    try:
        with open(path, 'r') as f:
            data = yaml.safe_load(f)
    except Exception as e:
        print(f"Skipping {fname}: parse error: {e}")
        continue
    if not data or 'on' not in data:
        continue
    # Overwrite triggers
    data['on'] = standard_triggers
    with open(path, 'w') as f:
        yaml.dump(data, f, default_flow_style=False)
    print(f"Updated triggers in {fname}")

# Known SPC Cases Fixture
# Ground-truth SPC values for synthetic generators
# These files define the expected SPC range for each generator

[SPC=0.0]
type = "uniform_random"
description = "Uniform random action selection — no predictive structure"
expected_spc = 0.0
tolerance = 0.05
action_space_size = 5
n_steps = 10000
seed = 42

[SPC=0.5]
type = "50_percent_calibrated"
description = "50% accurate predictor — random coin flip on half the actions"
expected_spc = 0.5
tolerance = 0.05
action_space_size = 5
n_steps = 10000
seed = 42

[SPC=1.0]
type = "perfect_predictor"
description = "Perfectly predicts every action"
expected_spc = 1.0
tolerance = 0.01
action_space_size = 5
n_steps = 10000
seed = 42

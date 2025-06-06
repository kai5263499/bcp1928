# Define the scripts directory
SCRIPTS_DIR := scripts

# Get a list of all scripts in the directory
SCRIPTS := $(wildcard $(SCRIPTS_DIR)/*.sh)

# Get a list of all script names without the directory and extension
SCRIPT_NAMES := $(basename $(notdir $(SCRIPTS)))

# Define a rule for each script
$(SCRIPT_NAMES): %: $(SCRIPTS_DIR)/%.sh
	@echo "Running script: $<"
	@./$<

# Specify the scripts as dependencies for the default target
all: $(SCRIPT_NAMES)
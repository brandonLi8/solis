# Copyright © 2022 Brandon Li. All rights reserved.

# ========================================================= #
#  Makefile for dev workflow commands. Cargo is still used
#  to run, test, and publish this project.
#
#  @author Brandon Li <brandon.li@berkeley.com>
# ========================================================= #

default: help

close-ticket:           ## Merges the current ticket branch into master.
	@echo ${INFO} Merging ${BRANCH}... ${NEWLINE}

	@# Check that the current branch is a ticket branch.
	@if [[ ! ${BRANCH} =~ ^(feature|patch|maintenance)/(.+)/[0-9]+$$ ]]; then \
		echo ${FAIL} Not on a ticket branch.; exit 1; \
	fi

	@# Must merge from a clean status.
	@if [[ ! -z ${CHANGES} ]]; then \
		echo ${FAIL} Not on a clean status: ${NEWLINE}"${CHANGES}"${NEWLINE}; exit 1; \
	fi

	@git checkout master
	@git merge --squash ${BRANCH}
	@git commit --no-edit
	@git commit --amend -m "(closes #${ISSUE} - ${BRANCH})" -m "$$(git log --format=%B -n1)" -n
	@git push
	@git branch -D ${BRANCH}
	@git push origin --delete ${BRANCH}

	@echo ${NEWLINE}${DONE}


ticket:                 ## Creates a ticket branch.\n
                        ##
                        ## Usage: (see docs/maintenance-guide.md for more info)
                        ## make ticket type={feature|patch|maintenance} name={.*} issue={[0-9]+}\n
	@# Validate type argument.
	@if [[ ! '$(type)' =~ ^(feature|patch|maintenance)$$ ]]; then \
		[ -z '$(type)' ] && \
			echo ${NEWLINE}${FAIL} type argument not provided. || \
			echo ${NEWLINE}${FAIL} Invalid type argument: ${bold}$(type)'${reset}' ; \
			echo Run ${blue_fg}make ticket type='{feature|patch|maintenance}' name={.*} issue={[0-9]+}'${reset}'${NEWLINE}; \
			exit 1; \
	fi

	@# Validate name.
	@if [ -z '$(name)' ]; then \
			echo ${NEWLINE}${FAIL} name argument not provided. || \
			echo Run ${blue_fg}make ticket type='{feature|patch|maintenance}' name={.*} issue={[0-9]+}'${reset}'${NEWLINE}; \
			exit 1; \
	fi

	@# Validate issue.
	@if [[ ! '$(issue)' =~ ^[0-9]+$$ ]]; then \
		[ -z '$(issue)' ] && \
			echo ${NEWLINE}${FAIL} issue argument not provided. || \
			echo ${NEWLINE}${FAIL} Invalid issue argument: ${bold}$(issue)'${reset}' ; \
			echo Run ${blue_fg}make ticket type='{feature|patch|maintenance}' name={.*} issue={[0-9]+}'${reset}'${NEWLINE}; \
			exit 1; \
	fi

	@git checkout master
	@git pull
	@git checkout -b "$(type)/$(name)/$(issue)"
	@git push -u origin "$(type)/$(name)/$(issue)"


update-copyright:       ## Updates the copyright statements of every file in the project.\n
	@echo ${INFO} Updating copyrights... ${NEWLINE}

	@#Loop through files that aren't in git-ignore
	@for file in $(shell git ls-files); do \
		# Ignore files that don't have copyright comments.  \
		if [[ $${file} =~ ${NO_COPYRIGHT_FILE_MATCH} ]]; then continue; \
		\
		# Search the first line for COPYRIGHT_MATCH. \
		elif [ "$$(sed -n -E '1{/${COPYRIGHT_MATCH}/p};q' $${file})" ]; then \
			# Replace the date with the correct date \
			sed -i -E '1 s/${COPYRIGHT_DATE_MATCH}/${COPYRIGHT_START_YEAR}-${CURRENT_YEAR}/' $${file}; \
		# Warn of files that are missing or have invalid copyright comments. \
		else \
			echo ${WARN} skipped $${file}; \
		fi \
	done

	@echo ${NEWLINE}${DONE}


help:                   ## Auto-prints documentation.
	@echo; sed -ne '/@sed/!s/## //p' $(MAKEFILE_LIST) | sed -E 's/\\n$$/\n/g';

# ========================================================= #

COPYRIGHT_START_YEAR = 2022
BRANCH = $(shell git symbolic-ref --short HEAD)
ISSUE = $(shell echo ${BRANCH} | sed -E 's/^(feature|patch|maintenance)\/(.+)\///')
CHANGES = $$(git status --porcelain)
CURRENT_YEAR = $(shell date +%Y)
COPYRIGHT_DATE_MATCH = ([0-9]){4,}(-([0-9]){4,})?
COPYRIGHT_MATCH = Copyright © ${COPYRIGHT_DATE_MATCH} Brandon Li. All rights reserved\.
NO_COPYRIGHT_FILE_MATCH = .*\.(jpg|svg|png|mp4|ico|json|browserslistrc)$

bold = $(shell tput bold)
black_bg = $(shell tput setab 0)
black_fg = $(shell tput setaf 0)
red_bg = $(shell tput setab 1)
red_fg = $(shell tput setaf 1)
green_bg = $(shell tput setab 2)
green_fg = $(shell tput setaf 2)
yellow_bg = $(shell tput setab 3)
yellow_fg = $(shell tput setaf 3)
blue_bg = $(shell tput setab 4)
blue_fg = $(shell tput setaf 4)
magenta_bg = $(shell tput setab 5)
magenta_fg = $(shell tput setaf 5)
cyan_bg = $(shell tput setab 6)
cyan_fg = $(shell tput setaf 6)
white_bg = $(shell tput setab 7)
white_fg = $(shell tput setaf 7)
reset = $(shell tput sgr0)

INFO = '${blue_bg}${black_fg} INFO ${reset}'
WARN = '${yellow_bg}${black_fg} WARN ${reset}'
FAIL = '${red_bg}${black_fg} FAIL ${reset}'
DONE = '${green_bg}${black_fg} DONE ${reset}'
CHECK = '${green_fg}\xE2\x9C\x94${reset}'
NEWLINE=$$'\n'
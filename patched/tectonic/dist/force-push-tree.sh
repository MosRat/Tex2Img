#! /bin/bash
# Copyright 2019 The Tectonic Project
# Licensed under the MIT License.

# A helper script to deploy a tree of files to another Git repository by
# copying them, amend-committing, and force-pushing. The main use case for
# this script is to deploy generated HTML trees to make them available using
# GitHub pages.
#
# Arguments:
#
# $1 - path to the file tree to be deployed
# $2 - HTTPS URL of the GitHub repository that will receive the files
# $3 - path within the destination Git repository where the tree will land
# $4 - brief free text identifying the commit/version of what's being deployed
#      (used in the Git logs)
#
# We assume that GitHub commit creation and push authentication have been set up
# externally.

set -e

# Parameters

src_path="$1"
dest_repo_url="$2"
dest_repo_path="$3"
commit_desc="$4"

# Set up the target repo.

echo "Cloning target repository $dest_repo_url ..."
tmprepo="$(mktemp -d)"
git clone "$dest_repo_url" "$tmprepo"
mkdir -p "$tmprepo/$dest_repo_path"
rm -rf "$tmprepo/$dest_repo_path"

# Update the HTML and commit.

echo "Committing and pushing changes ..."
cp -a "$src_path" "$tmprepo/$dest_repo_path"

pushd "$tmprepo"
git add "$dest_repo_path"
git commit --amend -m "Most recent update: $dest_repo_path - $commit_desc"
git push -f
popd

# And that's it.
echo "Success."

#/usr/bin/bash
#
#This script initializes a new day for Advent of Code 

YEAR=2023
DAY=$1

if [ -z "$DAY" ]; then
  echo "Specify a day to initialize"
  echo "Usage: $0 <day>"
  exit 1
fi

echo "Copying template"

NEW_DAY="day-$DAY"
cp -r template/ $NEW_DAY/

echo "Updating template"
# Change the project name in template folder Cargo.toml
sed -i 's/^name = "template"$/name = "'$NEW_DAY'"/' $NEW_DAY/Cargo.toml

# Change into the src directory
cd $NEW_DAY/src

# Download the input file for the day
source .aoc_session
echo "Downloading input file"
curl -s -o input.txt https://adventofcode.com/$YEAR/day/${DAY}/input --cookie "session=$AOC_SESSION"

# Start editing the source file
nvim .

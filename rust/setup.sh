# curls the aoc input and saves it to a file in the proper directory
#

# get the day
#
COOKIE="session=53616c7465645f5f120650f85bfffc32f88618862d2c9b8ec7b1bc8cf94fc847fafecbc6256cd005aaed05a009397893e43b6feb71bbc899bda426dfc3aa2c0a"

if [ -z "$1" ]
then
    echo "Usage: setup.sh <day>"
    exit 1
fi

day=$1

# if the day directory doesn't exist, make it
mkdir -p $day

# cd into the directory and cargo new it
cd $day
cargo new --bin aoc_$day

# make the input directory for the new day
mkdir -p aoc_$day/input

# get the input
curl https://adventofcode.com/2022/day/$day/input --cookie $COOKIE > aoc_$day/input/input.txt



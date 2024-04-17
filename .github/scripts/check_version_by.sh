# Inputting version should has no prefix 'v'
input_version=$1

echo_out=false

function print() {
  if $echo_out; then echo "$1"; fi
}

if [[ -z "$2" ]]; then
  latest=$(git tag -l | sed -nE "s/^(v([0-9]+)\.([0-9]+)\.([0-9]+)[-._]?(([a|A])lpha|([b|B])eta)?([0-9]*))$/\2.\3.\4.\8\6\7 \1 \2.\3.\4 \5 \8/p" | sort --version-sort -r -s -k1,1 | head -1)
  echo_out=true
else
  latest=$(echo "$2" | sed -nE "s/^(v?([0-9]+)\.([0-9]+)\.([0-9]+)[-._]?(([a|A])lpha|([b|B])eta)?([0-9]*))$/\2.\3.\4.\8\6\7 \1 \2.\3.\4 \5 \8/p")
  echo_out=false
fi
arr=($latest)
tag=${arr[1]}
version=${arr[2]}
tailing=${arr[3]}
tailing_version=${arr[4]}

print "Package version: $input_version"
print "Target version: $tag ($version, $tailing, $tailing_version)"

if [[ ${#input_version} -eq ${#version} ]] && [[ -z "$tailing" ]] && [[ -z "$tailing_version" ]] && [[ "$input_version" = "$version" ]]; then
  # YES
  print "Matched!"
  exit 0
fi

REGEX="^$version[-._]$tailing[.0]*$tailing_version$"
if [[ ! $input_version =~ $REGEX ]]; then
  # NO
  print "Not match!"
  exit 1
fi

# YSE
print "Matched!"
exit 0
#!/usr/bin/env bash

cat <<EOF
                  _      _     
                 (_)    | |    
  ___  ____ _   _ _  ___| |__  
 /___)/ _  | | | | |/___)  _ \ 
|___ | |_| | |_| | |___ | | | |
(___/ \__  |____/|_(___/|_| |_|
         |_|

EOF
pass_count=0
fail_count=0

tests=(
    "Simple test|squish input.png -o /var/tmp/output.png -v|0"
    "Provide width|squish input.png -w 600 -o /var/tmp/output.png|0"
    "Blur image|squish input.png -b 4 -o /var/tmp/output.png|0"
    "Absent file|squish absent.png -o /var/tmp/output.png|1"
    "Incorrect input file|squish test.sh -o /var/tmp/output.png|1"
)

for test in "${tests[@]}"; do
    IFS='|' read -r title cmd expected_exit_code <<<"$test"

    echo "> $title"
    echo "$cmd"
    echo
    eval "$cmd"
    exit_code=$?
    if [ $exit_code -eq $expected_exit_code ]; then
        echo "✅ command behaves as expected"
        ((pass_count++))
    else
        echo "❌ command returned $exit_code, expected $expected_exit_code"
        ((fail_count++))
    fi
    echo
    echo "==============================="
    echo
done

echo "Summary:"
echo "- Passed: $pass_count"
echo "- Failed: $fail_count"

if [ $fail_count -gt 0 ]; then
    exit 1
else
    exit 0
fi

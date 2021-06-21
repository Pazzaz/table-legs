#!/bin/bash
counter=0
filename=$1
while read -r line
do
  echo "$line /(144^$counter)" |  bc -l
  let counter=$counter+1
done


#!/bin/sh
x=1
while [ $x -le 9 ];
do
  echo $x
  (( x++ ))
done

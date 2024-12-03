import math
import re

from getinput import getall

mul_re = re.compile(r"mul\(\d+,\d+\)")
do_re = re.compile(r"do\(\)")
dont_re = re.compile(r"don't\(\)")
all_re = re.compile(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))")

s = getall()

tot = 0
do = True
last = None

for kw in all_re.findall(s):
    if kw[1]:
        do = True
    elif kw[2]:
        do = False
    elif do and (mul := kw[0]):
        tot += math.prod(int(n) for n in mul[4:-1].split(","))

print(tot)

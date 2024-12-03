import math
import re

from getinput import getall

regex = re.compile(r"mul\(\d+,\d+\)")

s = getall()
muls = regex.findall(s)

tot = sum(math.prod(int(n) for n in mul[4:-1].split(",")) for mul in muls)

print(tot)

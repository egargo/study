#!/usr/bin/env python3

# https://www.codewars.com/kata/58b20418e39e2ad9ae000150/train/python


import re


def sum_accounts(text):
	numbers = re.findall(pattern=r"(\$[0-9]{1,})|(\-\$[0-9]{1,})", string=text)
	minuend = [int(i[0].replace("$", "")) for i in numbers if i[0] != ""]
	subtrahend = [int(i[1].replace("-$", "")) for i in numbers if i[1] != ""]

	return sum(minuend) - sum(subtrahend)


if __name__ == "__main__":
	text = 'wbbpn"G_b-$24.-$99.M_6a2`GK-$73 $234,$70 9r!+U-KHY8daz<q+r<szobZv"\'FD<>wh_+,Qf,Tz>iw1v~#Z6`kl~\'tvK!+bnz5;HAmd+8f7s3PsDKtzeZ,UxMaopkmS'
	print(sum_accounts(text))

# https://www.codewars.com/kata/587a75dbcaf9670c32000292/train/python


def filter_words(st):
    return " ".join([i for i in st.capitalize().split(" ") if i != ""])


def new_arg(title: str):
    print(f"new_arg: {title}")
    return title


def consume_args(v1: str, v2: str):
    print(f"consume_args: {v1} and {v2}")


if __name__ == "__main__":
    v1 = new_arg("v1")
    v2 = new_arg("v2")
    consume_args(v1, v2)
    # new_arg: v1
    # new_arg: v2
    # consume_args: v1 and v2

    consume_args(new_arg("v1"), new_arg("v2"))
    # new_arg: v1
    # new_arg: v2
    # consume_args: v1 and v2

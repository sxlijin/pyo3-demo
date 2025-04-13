import my_rust_crate


def main():
    print("Hello from my-python-app!")
    # my_rust_crate.start_server_via_tokio("127.0.0.1:3000")
    obj = my_rust_crate.MyRustObject.my_static_method("Hello from Python!")
    obj2 = my_rust_crate.MyRustObject.my_static_method("Hello from Python!")
    d = {obj: "Hello from Python!", obj2: "Hello from Python2!"}
    print(d)


if __name__ == "__main__":
    main()

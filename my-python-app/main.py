import my_rust_crate


def main():
    print("Hello from my-python-app!")
    my_rust_crate.start_server_via_tokio("127.0.0.1:3000")


if __name__ == "__main__":
    main()

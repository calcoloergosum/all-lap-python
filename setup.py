"""Setup dino-py"""
from setuptools import setup
from setuptools_rust import RustExtension


def main():
    setup_requires = ["setuptools-rust>=0.12", "wheel", "setuptools_scm"]
    install_requires = []
    tests_require = install_requires

    setup(
        name="all-lap",
        use_scm_version={
            'write_to': 'all_lap/version.py',
            'write_to_template': '__version__ = "{version}"\n'
        },
        classifiers=[
            "Programming Language :: Python",
            "Programming Language :: Rust",
            "Operating System :: POSIX",
            "Operating System :: MacOS :: MacOS X",
        ],
        packages=["all_lap"],
        rust_extensions=[
            RustExtension("all_lap.rust_ext", "Cargo.toml", debug=False),
        ],
        author='Han Jaeseung',
        author_email='han@aisystemresearch.com',
        install_requires=install_requires,
        tests_require=tests_require,
        setup_requires=setup_requires,
        include_package_data=True,
        zip_safe=False,
    )


if __name__ == "__main__":
    main()
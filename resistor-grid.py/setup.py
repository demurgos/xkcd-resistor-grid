from setuptools import setup
from os import path

setup(name="ResistorGrid",
    description="A library to reduce electrical circuits to equivalent resistors",
    long_description=open(path.join(path.abspath(path.dirname(__file__)), "README.md"), "r").read(),
    author="Charles Samborski",
    packages=["resistor_grid"],
    install_requires=["docutils", "pygments", "pypiwin32", "kivy.deps.sdl2", "kivy.deps.glew", "kivy.deps.gstreamer", "kivy"],
    classifiers=["Development Status :: 3 - Alpha"])

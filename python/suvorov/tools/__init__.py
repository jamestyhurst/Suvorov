"""Tooling that supports game content under ``games\\``.

These modules are not part of the simulation core. They read and check content files; they
never execute game rules. They live outside ``suvorov.core`` so that the core stays free of
anything to do with files, formats, or the command line.
"""

#! /usr/bin/env python3

import os
import subprocess

schemadir = "/usr/share/glib-2.0/schemas"

if not os.environ.get('DESTDIR'):
    print ('Compiling the gsettings schemas...')
    subprocess.call (['glib-compile-schemas', schemadir])

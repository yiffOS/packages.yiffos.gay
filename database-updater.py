# Built in modules
import os
import shutil
import sqlite3
from struct import pack
import psycopg
import json

# Pip modules
from git import Repo

###############################################################################

def lst2pgarr(alist):
    return "'" + '{' + ','.join(alist) + '}' + "'"

def run_update_insert(package):
    cur = conn.cursor()
    
    sql_insert  = """
    INSERT INTO public.packages (name, version, epoch, description, groups, url, license, depends, optional_depends,
                make_depends, provides, conflicts, replaces, maintainers, repo)
                
    VALUES ('{name}', '{version}', {epoch}, '{description}', {groups},
            '{url}', {license}, {depends},
            {optional_depends}, {make_depends}, {provides}, {conflicts}, {replaces}, {maintainers}, '{repo}')

    ON CONFLICT (name) DO UPDATE
    SET (name, version, epoch, description, groups, url, license, depends, optional_depends, make_depends, provides, conflicts, replaces, maintainers, repo) = (EXCLUDED.name, EXCLUDED.version, EXCLUDED.epoch, EXCLUDED.description, EXCLUDED.groups, EXCLUDED.url, EXCLUDED.license, EXCLUDED.depends, EXCLUDED.optional_depends, EXCLUDED.make_depends, EXCLUDED.provides, EXCLUDED.conflicts, EXCLUDED.replaces, EXCLUDED.maintainers, EXCLUDED.repo);"""
    
    package["description"] = package["description"].replace("'", "''")
    
    if len(package["groups"]) == 0:
        package["groups"] = 'NULL'
    else:
        package["groups"] = lst2pgarr(package["groups"])
        
    if len(package["depends"]) == 0:
        package["depends"] = 'NULL'
    else:
        package["depends"] = lst2pgarr(package["depends"])
                
    if len(package["optional_depends"]) == 0:
        package["optional_depends"] = 'NULL'
    else:
        package["optional_depends"] = lst2pgarr(package["optional_depends"])
            
    if len(package["make_depends"]) == 0:
        package["make_depends"] = 'NULL'
    else:
        package["make_depends"] = lst2pgarr(package["make_depends"])
               
    if len(package["conflicts"]) == 0:
        package["conflicts"] = 'NULL'
    else:
        package["conflicts"] = lst2pgarr(package["conflicts"])
                
    if len(package["replaces"]) == 0:
        package["replaces"] = 'NULL'
    else:
        package["replaces"] = lst2pgarr(package["replaces"])
            
    cur.execute(sql_insert.format(
        name = package["name"],
        version = package["version"],
        epoch = package["epoch"],
        description = package["description"],
        groups = package["groups"],
        url = package["url"],
        license = lst2pgarr(package["license"]),
        depends = package["depends"],
        optional_depends = package["optional_depends"],
        make_depends = package["make_depends"],
        provides = lst2pgarr(package["provides"]),
        conflicts = package["conflicts"],
        replaces = package["replaces"],
        maintainers = lst2pgarr(package["maintainers"]),
        repo = "core"
    ))
    
###############################################################################

### Setting up the environment
print("==> Setting up the environment...")

print("=> Connecting to the database...")
conn = psycopg.connect(os.environ.get("DATABASE_URL"))
print("> Database version: ")
print(conn.cursor().execute("SELECT version();").fetchone())

# Clone the repo
print("=> Cloning PKGSCRIPT repo...")
repo = Repo.init(os.path.join(os.getcwd(), "PKGSCRIPT"))

origin = repo.create_remote("origin", os.environ.get("REPO_URL"))
origin.fetch()

repo.create_head("main", origin.refs.main).set_tracking_branch(origin.refs.main).checkout()

### Scan through all folders in the repo
print("==> Scanning through all folders in the repo...")

for f in os.listdir(os.path.join(os.getcwd(), "PKGSCRIPT")):
    if os.path.isdir(os.path.join(os.getcwd(), "PKGSCRIPT", f)):
        
        # Check if there is a PKGINFO file
        if os.path.isfile(os.path.join(os.getcwd(), "PKGSCRIPT", f, "PKGINFO")):        
                
            # Read the PKGINFO file\
            with open(os.path.join(os.getcwd(), "PKGSCRIPT", f, "PKGINFO")) as pkg_info:
                pkg_info_content = json.load(pkg_info)
                
                print("=> " + pkg_info_content["name"])
                
                run_update_insert(pkg_info_content)        

conn.commit()
conn.close()

## Clean up
print("==> Cleaning up...")
shutil.rmtree(os.path.join(os.getcwd(), "PKGSCRIPT"))

###############################################################################
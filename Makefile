MODULES = cyphera_udf
EXTENSION = cyphera
DATA = cyphera--1.0.sql
CONTROL = cyphera.control

PG_CONFIG = pg_config
PGXS := $(shell $(PG_CONFIG) --pgxs)
include $(PGXS)

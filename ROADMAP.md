# Roadmap

## v0.1 - Static Cash flow - API

The application shall allow to view the static cash flow. This means, it shall summarize all incoming flows (currently only salary) and outgoing
flows (e.g. abos, insurance, etc.). This shall be presented like financial results in tabular manner. Therefore, the API shall provide the following
features:
* **HTML**
    + *C*reate a new cash flow (`example.com/finances/add`)
    + (*R*) Show financial results (`example.com/finances`)
    + (*R*) Show cash flow details (`example.com/finances/<id>`)
    + *U*pdate a cash flow (via button on show details)
    + *D*elete a cash flow (via button on show all)
* **REST**
    + *C*reate a new cash flow (`POST` - `example.com/finances/add`)
    + *R*ead all cash flows (`GET`- `example.com/finances`)
    + *R*ead cash flow details (`GET`- `example.com/finances/<id>`)
    + *U*pdate a cash flow (`PUT`- `example.com/finances/<id>`)
    + *D*elete a cash flow (`DELETE` - `example.com/finances/<id>`)

## v0.2 - Static cash flow - Database

For each api call (Create, Read, Update, Delete) a database call shall be implemented. The database software is MongoDB and for the first
implementation a local database shall be used. No authentication, just simlpe and easy. CRUD calls are:
* *C*reate cash flow
* *R*ead a single cash flow details (by its id)
* *R*ead all cash flows (name, value)
* *U*pdate cash flow
* *D*elete cash flow
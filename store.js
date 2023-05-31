const TEMPLATE = {
    "variables": [
        // {
        //     "name": ""
        //     "value": ""
        // }
    ]
};

class Store {
    constructor() {
        let found = localStorage.getItem("storeVariables");
        // if it exists, exit constructor
        if (found != null) {
            return
        }
        // create the entry
        localStorage.setItem("storeVariables", JSON.stringify(TEMPLATE));
    }

    setVariable(name, value) {
        let variables = JSON.parse(localStorage.getItem("storeVariables"));
        let variable = {
            "name": name,
            "value": value,
        }
        let exists = this.checkIfExists(name);
        if (!isNaN(exists)) {
            variables.variables[exists] = variable;
        } else {
            variables.variables.push(variable);
        }
        localStorage.setItem("storeVariables", JSON.stringify(variables))
        return value;
    }

    getVariable(name) {
        let variables = JSON.parse(localStorage.getItem("storeVariables"));
        let exists = this.checkIfExists(name);
        if (!isNaN(exists)) {
            return variables.variables[exists].value
        } else {
            return null;
        }
    }

    checkIfExists(name) {
        let variables = JSON.parse(localStorage.getItem("storeVariables"));
        let index = 0;
        for (const variable of variables.variables) {
            if (variable.name == name) {
                return index;
            }
            index =+ 1;
        }
        return 'false';
    }

    debugPrintVariables() {
        let variables = JSON.parse(localStorage.getItem("storeVariables"));
        console.log(variables.variables);
    }
}
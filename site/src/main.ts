import './style.css'
import {GarnishScript} from "browser_garnish";

let script = new GarnishScript("main", "");

const outputValueTemplate = document.getElementById("outputValueTemplate") as HTMLTemplateElement;
const outputList = document.getElementById("outputList");

let sourceArea = document.getElementById("sourceArea");
sourceArea.addEventListener("keydown", (e) => {
    if (e.code === "Tab") {
        e.preventDefault();
        e.stopPropagation();
        let area = e.target as HTMLTextAreaElement;
        area.value = area.value + "  ";
        script.set_text(area.value);
    }
});

sourceArea.addEventListener("input", (e) => {
    let area = e.target as HTMLTextAreaElement;
    script.set_text(area.value);
});

let inputArea = document.getElementById("inputArea");
inputArea.addEventListener("keydown", (e) => {
    if (e.code === "Tab") {
        e.preventDefault();
        e.stopPropagation();
        let area = e.target as HTMLTextAreaElement;
        area.value = area.value + "  ";
        script.set_input(area.value);
    }
});

inputArea.addEventListener("input", (e) => {
    let area = e.target as HTMLTextAreaElement;
    script.set_input(area.value);
});

document.getElementById("executeButton").addEventListener("click", (e) => {
    script.compile();
    if (script.get_error() !== undefined) {
        console.error(script.get_error());
        return;
    }

    script.execute();
    if (script.get_error() !== undefined) {
        console.error(script.get_error());
        return;
    }

    let result = script.get_execution_result(script.get_execution_count() - 1);

    let valueElement = outputValueTemplate.content.cloneNode(true) as DocumentFragment;
    let item = valueElement.querySelector("li");
    item.innerText = result;

    outputList.appendChild(item);
});

document.getElementById("clearButton")?.addEventListener("click", (e) => {
    script.clear_executions();

    while (outputList.lastElementChild) {
        outputList.removeChild(outputList.lastElementChild);
    }
})
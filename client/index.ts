import type Interop from "./interop";

import { AutomaticConnection } from "./AutomaticConnection";

const c = new AutomaticConnection("bob");

const messages = document.getElementById("messages")!;
const app = document.getElementById("app")!;

const responseCreatedOk = (id: string) => {
  ws.send(
    JSON.stringify({
      CreatedOk: { id },
    })
  );
};

const responseRemovedOk = () => {
  ws.send(JSON.stringify({ RemovedOk: null }));
};

function handleCommand(cmd: Interop.Command) {
  if ("CreateElement" in cmd) {
    const args = cmd.CreateElement;
    console.log("create-element", args);

    const el = document.createElement(args.el);
    el.id = String(Math.floor(Math.random() * 10e4));
    if (args.attrs) {
      Object.entries(args.attrs).forEach(([attr, value]) => {
        if (el.hasAttribute(attr)) {
          el.setAttribute(attr, value);
        } else {
          Object.assign(el, { attr: value });
        }
      });
    }
    const parent: HTMLElement =
      (args.parent && document.getElementById(args.parent)) || app;

    parent.appendChild(el);
    responseCreatedOk(el.id);
    return true;
  } else if ("RemoveElement" in cmd) {
    const args = cmd.RemoveElement;
    const toRemove = document.getElementById(args.id);
    if (toRemove) {
      toRemove.remove();
      responseRemovedOk();
    }

    return true;
  }
  return false;
}

function print(...args: any[]) {
  console.log(...args);
  const pre = document.createElement("pre");
  pre.textContent = args.join("");
  messages.appendChild(pre);
  messages.scrollTop = messages.scrollHeight;
}

print("tryConnect");
const ws = new WebSocket("ws://localhost:3012");
ws.onmessage = ({ data }) => {
  if (data.startsWith("json")) {
    const json = data.substr(4);
    const cmd = JSON.parse(json);

    if (handleCommand(cmd)) {
      return;
    }
  }
  print(`server responded: "${data.replace(/\"/g, '\\"')}"`);
};
ws.onopen = () => {
  print("ws connection open");
  ws.send("hi from client!");
  print("message sent!");
};
ws.onclose = () => {
  setTimeout(() => {
    window.location.reload();
  }, 1000);
};

const label = document.createElement("label");
label.setAttribute("for", "edit");
label.innerText = "search for stuff";
const edit = document.createElement("input");
edit.id = "edit";
edit.type = "text";
edit.oninput = (e) => ws.send((e.target as typeof edit).value);

app.insertBefore(edit, messages);
app.insertBefore(label, edit);

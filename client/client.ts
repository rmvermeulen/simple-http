import { AutomaticConnection } from "./AutomaticConnection";

const messages = document.getElementById("messages");
const app = document.getElementById("app");

const responseCreatedOk = (id) => {
  ws.send(
    JSON.stringify({
      CreatedOk: { id },
    })
  );
};

const responseRemovedOk = () => {
  ws.send(
    JSON.stringify({
      RemovedOk: null,
    })
  );
};

function handleCommand(cmd) {
  const [key] = Object.keys(cmd);
  switch (key) {
    case "CreateElement": {
      const args = cmd[key];
      console.log("create-element", args);

      const el = document.createElement(args.el);
      el.id = Math.floor(Math.random() * 10e4);
      if (args.attrs) {
        Object.entries(args.attrs).forEach(([attr, value]) => {
          if (el.hasAttribute(attr)) {
            el.setAttribute(attr, value);
          } else el[attr] = value;
        });
      }
      const parent: HTMLElement = args.parent
        ? document.getElementById(args.parent)
        : app;
      parent.appendChild(el);
      console.log({
        created: el,
        parent,
      });
      responseCreatedOk(el.id);
      return true;
    }
    case "RemoveElement": {
      console.log("remove-element");
      const args = cmd[key];
      const toRemove = document.getElementById(args.id);
      if (toRemove) {
        console.log("removing", toRemove);
        toRemove.parentNode.removeChild(toRemove);
        responseRemovedOk();
      }

      return true;
    }
  }
  return false;
}

function print(...args) {
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
label.for = "edit";
label.innerText = "search for stuff";
const edit = document.createElement("input");
edit.id = "edit";
edit.type = "text";
edit.oninput = (e) => ws.send(e.target.value);

app.insertBefore(edit, messages);
app.insertBefore(label, edit);

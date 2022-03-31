import { Actions, Attributes, Recipe, Status } from "./Craft";
import { emit, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

const create_solver = (s: Status) => {
  return invoke("create_solver", { status: s });
};

const init_solver = (s: Status, allowedList: Actions[]) => {
  return invoke("init_solver", { status: s, allowedList });
};

const read_solver = (s: Status): Promise<Actions[]> => {
  return invoke("read_solver", { status: s });
};

export { create_solver, init_solver, read_solver };

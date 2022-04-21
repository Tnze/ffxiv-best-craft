import { Actions, Attributes, Recipe, Status } from "./Craft";
import { emit, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

const create_solver = (
  s: Status,
  useMuscleMemory: boolean,
  useManipulation: boolean
) => {
  return invoke("create_solver", {
    status: s,
    useMuscleMemory,
    useManipulation,
  });
};

const destroy_solver = (s: Status) => {
  return invoke("destroy_solver", { status: s });
};

const read_solver = (s: Status): Promise<Actions[]> => {
  return invoke("read_solver", { status: s });
};

export { create_solver, read_solver, destroy_solver };

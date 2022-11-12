import { Actions, Attributes, Recipe, Status } from "./Craft";
import { emit, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

export const create_solver = (
  status: Status,
  useMuscleMemory: boolean,
  useManipulation: boolean
) => {
  return invoke("create_solver", {
    status,
    useMuscleMemory,
    useManipulation,
  });
};

export const destroy_solver = (status: Status) => {
  return invoke("destroy_solver", { status });
};

export const read_solver = (status: Status): Promise<Actions[]> => {
  return invoke("read_solver", { status });
};

export const rika_solve = (status: Status): Promise<Actions[]> => {
  return invoke("rika_solve", { status })
}
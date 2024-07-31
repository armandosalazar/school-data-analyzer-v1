import { Injectable } from "@angular/core";
import { from, Observable } from "rxjs";
import { invoke } from "@tauri-apps/api";

export interface Student {
  id: number;
  register: number;
  name: string;
}

@Injectable({
  providedIn: "root",
})
export class StudentService {
  constructor() {}

  getStudents(limit: number, offset: number): Observable<Student[]> {
    return from(invoke<Student[]>("get_students", { limit, offset }));
  }

  countStudents(): Observable<number> {
    return from(invoke<number>("count_students"));
  }
}

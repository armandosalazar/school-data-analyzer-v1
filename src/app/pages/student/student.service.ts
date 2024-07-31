import { Injectable } from "@angular/core";
import { from, Observable } from "rxjs";
import { invoke } from "@tauri-apps/api";

@Injectable({
  providedIn: "root",
})
export class StudentService {
  constructor() {}

  getStudents(): Observable<any> {
    return from(invoke<any>("get_students"));
  }
}

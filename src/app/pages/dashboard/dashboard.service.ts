import { Injectable } from "@angular/core";
import { from, Observable } from "rxjs";
import { invoke } from "@tauri-apps/api";

@Injectable({
  providedIn: "root",
})
export class DashboardService {
  constructor() {}

  uploadFile(path: string): Observable<string> {
    return from(invoke<string>("upload_file", { path }));
  }
}

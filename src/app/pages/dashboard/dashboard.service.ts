import { Injectable } from "@angular/core";
import { from, Observable } from "rxjs";
import { invoke } from "@tauri-apps/api";

export interface Response {
  success: boolean;
  message: string;
}

@Injectable({
  providedIn: "root",
})
export class DashboardService {
  constructor() {}

  uploadFile(path: string): Observable<Response> {
    return from(invoke<Response>("upload_file", { path }));
  }
}

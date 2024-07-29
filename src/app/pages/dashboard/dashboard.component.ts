import { Component, inject } from "@angular/core";
import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
import { ButtonModule } from "primeng/button";
import { DashboardService } from "./dashboard.service";

@Component({
  selector: "app-dashboard",
  standalone: true,
  imports: [ButtonModule],
  templateUrl: "./dashboard.component.html",
  styleUrl: "./dashboard.component.css",
})
export class DashboardComponent {
  dashboardService: DashboardService = inject(DashboardService);
  path: string = "";

  uploadFile(): void {
    open({
      multiple: false,
      filters: [
        {
          name: "Data",
          extensions: ["csv"],
        },
      ],
    }).then((selected) => {
      this.dashboardService
        .uploadFile(selected?.toString() ?? "")
        .subscribe((message: string) => {
          console.log(message);
        });
    });
  }
  // path: string = '';
  // async uploadFile(): Promise<void> {
  //   const selected = await open({
  //     multiple: false,
  //     filters: [
  //       {
  //         name: 'Data',
  //         extensions: ['csv'],
  //       },
  //     ],
  //   });
  //
  //   this.path = selected?.toString() ?? '';
  //   console.log(selected);
  //
  //   await invoke<void>('upload_file', { path: this.path });
  // }
}

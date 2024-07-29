import { Component, inject } from "@angular/core";
import { open } from "@tauri-apps/api/dialog";
import { ButtonModule } from "primeng/button";
import { DashboardService, Response } from "./dashboard.service";
import { MessageService } from "primeng/api";
import { ToastModule } from "primeng/toast";
import { TableModule } from "primeng/table";
import { ProgressBarModule } from "primeng/progressbar";
import { InputTextModule } from "primeng/inputtext";
import { DropdownModule } from "primeng/dropdown";

interface File {
  id: number;
  path: string;
  name: string;
  date: string;
}

@Component({
  selector: "app-dashboard",
  standalone: true,
  imports: [
    ButtonModule,
    ToastModule,
    TableModule,
    ProgressBarModule,
    InputTextModule,
    DropdownModule,
  ],
  templateUrl: "./dashboard.component.html",
  styleUrl: "./dashboard.component.css",
  providers: [DashboardService, MessageService],
})
export class DashboardComponent {
  dashboardService: DashboardService = inject(DashboardService);
  messageService: MessageService = inject(MessageService);

  files: File[] = [];
  path: string = "";
  processing: boolean = true;

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
      if (!selected) {
        this.messageService.add({
          severity: "error",
          summary: "Error",
          detail: "No file selected",
        });
        return;
      }
      this.processing = true;
      this.dashboardService.uploadFile(selected?.toString() ?? "").subscribe({
        next: (response: Response) => {
          this.messageService.add({
            severity: "success",
            summary: "Success",
            detail: response.message,
          });
        },
        error: (error: Response) => {
          this.messageService.add({
            severity: "error",
            summary: "Error",
            detail: error.message,
          });
        },
      });
      this.processing = false;
    });
  }
}

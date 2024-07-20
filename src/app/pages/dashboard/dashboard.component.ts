import { Component } from "@angular/core";
import { ButtonModule } from "primeng/button";

@Component({
  selector: "app-dashboard",
  standalone: true,
  imports: [ButtonModule],
  templateUrl: "./dashboard.component.html",
  styleUrl: "./dashboard.component.css",
})
export class DashboardComponent {
  path: string = "";
  openDatabase(): void {
    console.log("openDatabase");
  }
}

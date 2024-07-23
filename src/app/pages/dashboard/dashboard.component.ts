import { Component } from '@angular/core';
import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';
import { ButtonModule } from 'primeng/button';

@Component({
  selector: 'app-dashboard',
  standalone: true,
  imports: [ButtonModule],
  templateUrl: './dashboard.component.html',
  styleUrl: './dashboard.component.css',
})
export class DashboardComponent {
  path: string = '';
  async openDatabase(): Promise<void> {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: 'Data',
          extensions: ['csv'],
        },
      ],
    });

    this.path = selected?.toString() ?? '';
    console.log(selected);

    await invoke<void>('upload_file', { path: this.path });
  }
}

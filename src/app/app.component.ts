import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { MenuItem } from 'primeng/api';
import { PanelMenuModule } from 'primeng/panelmenu';
import { ButtonModule } from 'primeng/button';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet, ButtonModule, PanelMenuModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css',
})
export class AppComponent {
  items: MenuItem[] = [
    {
      label: 'Files',
      icon: 'pi pi-file',
      items: [
        {
          label: 'New',
          icon: 'pi pi-plus',
        },
        {
          label: 'Search',
          icon: 'pi pi-search',
        },
        {
          label: 'Print',
          icon: 'pi pi-print',
        },
      ],
    },
    {
      label: 'Sync',
      icon: 'pi pi-cloud',
      items: [
        {
          label: 'Import',
          icon: 'pi pi-cloud-download',
        },
        {
          label: 'Export',
          icon: 'pi pi-cloud-upload',
        },
      ],
    },
    {
      label: 'Sign Out',
      icon: 'pi pi-sign-out',
    },
  ];

  // greetingMessage = '';
  path: string = '';

  // greet(event: SubmitEvent, name: string): void {
  //   event.preventDefault();

  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   invoke<string>("greet", { name }).then((text) => {
  //     this.greetingMessage = text;
  //   });
  // }

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

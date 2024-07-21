import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { MenuItem } from 'primeng/api';
import { PanelMenuModule } from 'primeng/panelmenu';
import { ButtonModule } from 'primeng/button';
import { MenuModule } from 'primeng/menu';
import { BadgeModule } from 'primeng/badge';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [
    CommonModule,
    RouterOutlet,
    ButtonModule,
    PanelMenuModule,
    MenuModule,
    BadgeModule,
  ],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css',
})
export class AppComponent {
  items: MenuItem[] = [
    {
      label: 'Dashboard',
      icon: 'pi pi-home',
      route: '/',
    },
    {
      label: 'Data Analysis',
      icon: 'pi pi-chart-bar',
      items: [
        {
          label: 'Students',
          icon: 'pi pi-users',
          route: '/student',
        },
        {
          label: 'Teachers',
          icon: 'pi pi-user',
          route: '/teacher',
        },
      ],
    },
    {
      label: 'Reports',
      icon: 'pi pi-file',
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
      label: 'Settings',
      icon: 'pi pi-cog',
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

import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { ButtonModule } from 'primeng/button';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet, ButtonModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css',
})
export class AppComponent {
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

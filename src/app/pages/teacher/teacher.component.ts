import { Component, OnInit } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';
import { TableModule } from 'primeng/table';

interface Teacher {
  id: number;
  payfoll: number;
  name: string;
}

@Component({
  selector: 'app-teacher',
  standalone: true,
  imports: [TableModule],
  templateUrl: './teacher.component.html',
  styleUrl: './teacher.component.css',
})
export class TeacherComponent implements OnInit {
  teachers: Teacher[] = [];

  ngOnInit() {
    this.getTeachers();
  }

  async getTeachers() {
    const res = await invoke<Teacher[]>('get_teachers');
    console.log(res);
    this.teachers = res;
  }
}

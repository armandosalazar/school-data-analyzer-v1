import { Component, inject, OnInit } from '@angular/core';
import { TableModule } from 'primeng/table';
import { TeacherService } from './teacher.service';
import { IconFieldModule } from 'primeng/iconfield';
import { InputIconModule } from 'primeng/inputicon';
import { InputTextModule } from 'primeng/inputtext';
import { LazyLoadEvent } from 'primeng/api';
import { event } from '@tauri-apps/api';

interface Teacher {
  id: number;
  payfoll: number;
  name: string;
}

@Component({
  selector: 'app-teacher',
  standalone: true,
  imports: [
    TableModule,
    IconFieldModule,
    InputIconModule,
    InputTextModule,
    // TagModule,
  ],
  templateUrl: './teacher.component.html',
  styleUrl: './teacher.component.css',
})
export class TeacherComponent implements OnInit {
  teacherService = inject(TeacherService);
  totalRecords: number = 0;
  teachers: Teacher[] = [];

  ngOnInit() {
    this.getData();
  }

  async loadTeachers(event: any) {
    console.log(event);
    this.teachers = await this.teacherService.getTeachers(
      event.first,
      event.rows
    );
  }

  async getData() {
    this.totalRecords = await this.teacherService.countTeachers();
  }
}

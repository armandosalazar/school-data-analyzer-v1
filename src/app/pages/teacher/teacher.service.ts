import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api';

interface Teacher {
  id: number;
  payroll: number;
  name: string;
}

@Injectable({
  providedIn: 'root',
})
export class TeacherService {
  constructor() {}

  async countTeachers(): Promise<number> {
    return await invoke<number>('count_teachers');
  }

  async getTeachers(
    offset: number,
    pageSize: number,
    sortField: string,
    sortOrder: number,
    filters: object
  ): Promise<Teacher[]> {
    return await invoke<Teacher[]>('get_teachers', {
      offset,
      pageSize,
      sortOrder,
      sortField,
      filters: JSON.stringify(filters),
    });
  }
}

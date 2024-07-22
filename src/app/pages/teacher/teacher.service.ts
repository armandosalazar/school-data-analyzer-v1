import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api';

interface Teacher {
  id: number;
  payfoll: number;
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

  async getTeachers(offset: number, pageSize: number): Promise<Teacher[]> {
    return await invoke<Teacher[]>('get_teachers', { offset, pageSize });
  }
}

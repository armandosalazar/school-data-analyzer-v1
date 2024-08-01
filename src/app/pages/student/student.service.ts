import { Injectable } from "@angular/core";
import { from, Observable } from "rxjs";
import { invoke } from "@tauri-apps/api";

export interface Student {
  id: number;
  register: number;
  name: string;
  type: string;
}

export interface Grade {
  id: number;
  firstGrade: number;
  firstFaults: number;
  firstWeighing: number;
  secondGrade: number;
  secondFaults: number;
  secondWeighing: number;
  thirdGrade: number;
  thirdFaults: number;
  thirdWeighing: number;
  subjectCode: string;
  subjectName: string;
  teacherPayroll: number;
  teacherName: string;
  divisionCode: number;
  divisionName: string;
}

export interface Filter {
  name: string;
  value: string;
  matchMode: string;
}

@Injectable({
  providedIn: "root",
})
export class StudentService {
  constructor() {}

  getStudents(
    limit: number,
    offset: number,
    filters: Filter[],
  ): Observable<Student[]> {
    return from(
      invoke<Student[]>("get_students", {
        limit,
        offset,
        filters: JSON.stringify(filters),
      }),
    );
  }

  countStudents(): Observable<number> {
    return from(invoke<number>("count_students"));
  }

  getGradesByStudentId(studentId: number): Observable<Grade[]> {
    return from(invoke<Grade[]>("get_grades_by_student_id", { studentId }));
  }
}

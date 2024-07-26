import { Routes } from '@angular/router';
import { DashboardComponent } from './pages/dashboard/dashboard.component';
import { TeacherComponent } from './pages/teacher/teacher.component';
import { StudentComponent } from './pages/student/student.component';
import { DivisionComponent } from './pages/division/division.component';
import { SubjectComponent } from './pages/subject/subject.component';
import { SpecialityComponent } from './pages/speciality/speciality.component';

export const routes: Routes = [
  { path: '', component: DashboardComponent },
  { path: 'dashboard', component: DashboardComponent },
  { path: 'student', component: StudentComponent },
  { path: 'teacher', component: TeacherComponent },
  { path: 'division', component: DivisionComponent },
  { path: 'subject', component: SubjectComponent },
  { path: 'speciality', component: SpecialityComponent },
];
